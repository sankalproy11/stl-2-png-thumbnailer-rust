const fs = require("fs");
const path = require("path");
const { generateThumbnail } = require("../lib");

const stlDirectory = "./stl_files";
const outputDirectory = "./output_images";

// Ensure the output directory exists
if (!fs.existsSync(outputDirectory)) {
  fs.mkdirSync(outputDirectory, { recursive: true });
}

// Read all STL files from the directory
fs.readdir(stlDirectory, (err, files) => {
  if (err) {
    return console.error("Failed to read directory:", err);
  }

  files.forEach((file) => {
    if (path.extname(file) === ".stl") {
      const stlFilePath = path.join(stlDirectory, file);
      const outputPngPath = path.join(
        outputDirectory,
        path.basename(file, ".stl") + ".png"
      );

      generateThumbnail(stlFilePath, outputPngPath)
        .then(() => console.log(`Thumbnail generated for ${file}`))
        .catch((error) =>
          console.error(`Error generating thumbnail for ${file}:`, error)
        );
    }
  });
});
