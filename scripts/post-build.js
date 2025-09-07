import { execSync } from 'child_process';
import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

// DMG 후처리 함수
function fixDmg() {
  const dmgPath = path.join(__dirname, '..', 'src-tauri', 'target', 'release', 'bundle', 'dmg', 'lupin_0.1.0_aarch64.dmg');
  
  if (!fs.existsSync(dmgPath)) {
    console.log('DMG file not found, skipping post-processing');
    return;
  }

  console.log('Post-processing DMG file...');
  
  try {
    // DMG를 RW 모드로 변환
    const tempDmg = dmgPath.replace('.dmg', '_temp.dmg');
    execSync(`hdiutil convert "${dmgPath}" -format UDRW -o "${tempDmg}"`, { stdio: 'inherit' });
    
    // DMG 마운트
    const mountOutput = execSync(`hdiutil attach "${tempDmg}" -nobrowse -noautoopen`).toString();
    const mountPoint = mountOutput.split('\t').pop().trim();
    
    // .VolumeIcon.icns 숨기기
    if (fs.existsSync(`${mountPoint}/.VolumeIcon.icns`)) {
      execSync(`SetFile -a V "${mountPoint}/.VolumeIcon.icns"`);
      console.log('Hidden .VolumeIcon.icns successfully');
    }
    
    // DMG 언마운트
    execSync(`hdiutil detach "${mountPoint}"`);
    
    // 최종 DMG로 변환
    fs.unlinkSync(dmgPath);
    execSync(`hdiutil convert "${tempDmg}" -format UDZO -o "${dmgPath}"`, { stdio: 'inherit' });
    fs.unlinkSync(tempDmg);
    
    console.log('DMG post-processing completed!');
  } catch (error) {
    console.error('Error during DMG post-processing:', error.message);
  }
}

// 실행
fixDmg();