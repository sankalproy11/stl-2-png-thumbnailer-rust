const fs = require("fs");
const path = require("path");
const native = require("../native");

// Promisify fs.readFile and fs.writeFile for use in async functions
const readFileAsync = fs.promises.readFile;
const writeFileAsync = fs.promises.writeFile;

// Function to convert generateThumbnail from callback-based to promise-based
function generateThumbnail(stlFilePath, outputPngPath) {
  return new Promise((resolve, reject) => {
    native.generateThumbnail(stlFilePath, outputPngPath, (err, result) => {
      if (err) reject(err);
      else resolve(result);
    });
  });
}

// Function to handle the generation of a thumbnail
async function processSTLFile(stlFilePath, outputPngPath = null) {
  if (!outputPngPath) {
    // If no output path provided, save the PNG in the same directory as the STL file
    const directory = path.dirname(stlFilePath);
    const baseName = path.basename(stlFilePath, ".stl");
    outputPngPath = path.join(directory, `${baseName}.png`);
  }

  try {
    await generateThumbnail(stlFilePath, outputPngPath);
    console.log(`Thumbnail successfully generated at: ${outputPngPath}`);
  } catch (error) {
    console.error(`Error generating thumbnail: ${error}`);
  }
}

module.exports = {
  generateThumbnail,
  processSTLFile,
};
