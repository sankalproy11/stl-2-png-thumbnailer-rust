const { processSTLFile } = require("./lib"); // Adjust the path if necessary

const stlFilePath = "/Users/sankalproy/Downloads/3d models/Yellow_Dragon.stl";

// Run the processing function
processSTLFile(stlFilePath)
  .then(() => console.log("Thumbnail generation complete."))
  .catch((err) => console.error("Error generating thumbnail:", err));
