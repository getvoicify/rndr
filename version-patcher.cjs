const config = require('./src-tauri/tauri.conf.json');
const package = require('./package.json');
const path = require('path');
const fs = require('fs');

const { version } = package;

const isLowerThanBaseVersion = (version, baseVersion) => {
    const versionArr = version.split('.');
    const baseVersionArr = baseVersion.split('.');
    for (let i = 0; i < versionArr.length; i++) {
        if (parseInt(versionArr[i]) < parseInt(baseVersionArr[i])) {
            return true;
        }
    }
    return false;
}
const currentVersion = config.package.version;

if (isLowerThanBaseVersion(version, currentVersion)) {
    throw new Error('Version lower than current version');
}

const newConfig = {
    ...config,
    package: {
        ...config.package,
        version,
    }
};

console.log('Writing new config', newConfig);

// write config to file
fs.writeFileSync(path.join(__dirname, 'src-tauri/tauri.conf.json'), JSON.stringify(newConfig, null, 2));
