const config = require('./src-tauri/tauri.conf.json');
const path = require('path');
const fs = require('fs');

const [version] = process.argv.slice(2);

console.log('Patching version', version);
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
