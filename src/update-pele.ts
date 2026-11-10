#!/usr/bin/env bun

const version = Bun.argv[2];
if (!version || !/^\d+\.\d+\.\d+(-[0-9A-Za-z.-]+)?$/.test(version)) {
	console.error("usage: update-pele.ts VERSION (for example, 1.13.0)");
	process.exit(1);
}

const libFile = Bun.file(new URL("./lib.rs", import.meta.url));
const vendorFile = Bun.file(new URL("./vendor.rs", import.meta.url));
const [lib, vendor] = await Promise.all([libFile.text(), vendorFile.text()]);
const urls =
	lib.match(
		/https:\/\/cdn\.jsdelivr\.net\/npm\/pele@[^/" ]+\/dist\/pele\.global\.js/g,
	) ?? [];
const hashes = lib.match(/sha384-[A-Za-z0-9+/=]+/g) ?? [];
if (urls.length !== 1 || hashes.length !== 1) {
	throw new Error("expected one Pele CDN URL and SRI hash in src/lib.rs");
}

const start = "        // BEGIN PELE SCRIPT\n";
const end = "        // END PELE SCRIPT";
const startIndex = vendor.indexOf(start);
const endIndex = vendor.indexOf(end);
if (
	vendor.split(start).length !== 2 ||
	vendor.split(end).length !== 2 ||
	endIndex < startIndex
) {
	throw new Error("expected one marked script block in src/vendor.rs");
}

const url = `https://cdn.jsdelivr.net/npm/pele@${version}/dist/pele.global.js`;
const response = await fetch(url);
if (!response.ok) {
	throw new Error(`download failed: ${response.status} ${url}`);
}
const bundle = await response.arrayBuffer();
const hash = `sha384-${new Bun.CryptoHasher("sha384").update(bundle).digest("base64")}`;
const encoded = encodeURIComponent(
	new TextDecoder("utf-8", { fatal: true }).decode(bundle),
).replace(/%(24|2B|2C|3B|3D|3A|40|2F|3F)/g, decodeURIComponent);
const script = `        ${JSON.stringify(`<script src="data:text/javascript,${encoded}"></script>\n`)}\n`;

await Promise.all([
	Bun.write(libFile, lib.replace(urls[0], url).replace(hashes[0], hash)),
	Bun.write(
		vendorFile,
		vendor.slice(0, startIndex + start.length) +
			script +
			vendor.slice(endIndex),
	),
]);
console.log(`Updated Pele to ${version} (${hash})`);
