import { useState } from "react";
import { useFilePicker } from "use-file-picker";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import { Commands, ContainerFileType } from "./utils";
import { SelectedFiles, UseFilePickerConfig } from "use-file-picker/types";

function App() {
  const [processing, setProcessing] = useState(false);

  const params: UseFilePickerConfig = {
    accept: [".yaml", ".json"],
    onFilesSuccessfullySelected: async ({ plainFiles, filesContent }: SelectedFiles<ArrayBuffer>) => {
      console.log('onFilesSuccessfullySelected', plainFiles, filesContent);

      setProcessing(true);
      await Promise.all(filesContent.map(async (file) => {
        console.log("installing...");
        await invoke(Commands.InstallFile, {
          contents: file.content,
          fileType: file.name.endsWith(".json") ? ContainerFileType.Json : ContainerFileType.Yaml
        });
      }));
      setProcessing(false);
    }
  }

  const { openFilePicker, filesContent, loading } = useFilePicker(params);

  return (
    <div className="container">
      <h1>Thunder Manager</h1>

      <div>
        <button
          type="submit"
          onClick={() => openFilePicker()}
        >
          Select file
        </button>
        <br />
        {loading ? (<div>
          <p>Loading file...</p>
        </div>) : <></>}
        {filesContent.map((file, index) => (
          <div>
            <h2>{file.name}</h2>
            <div key={index}>{processing ? `Processing ${file.name}...` : "Done!"}</div>
            <br />
          </div>
        )
        )}
      </div>

      <p>{"Select a container file to use (.json or .yaml)"}</p>
    </div >
  );
}

export default App;
