import { useState } from "react";
import { useFilePicker } from "use-file-picker";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import { Commands, ContainerFileType } from "./utils";
import { SelectedFiles, UseFilePickerConfig } from "use-file-picker/types";
import { Alert, Box, Button, CircularProgress, Container, Typography } from "@mui/material";
import CheckIcon from '@mui/icons-material/Check';

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
    <Container maxWidth="sm">
      <Box sx={{ my: 4 }} textAlign="center">
        <Button
          type="submit"
          onClick={() => openFilePicker()}
        >
          Select file
        </Button>
        <br />
        {loading ? (<div>
          <p>Loading file...</p>
        </div>) : <></>}
        {filesContent.map((file, index) => (
          <div>
            <Typography variant="h5">{file.name}</Typography>
            {processing ? <>
              <CircularProgress />
              <Typography key={index}>Processing {file.name}...</Typography>
            </> : <Alert icon={<CheckIcon fontSize="inherit" />} severity="success">
              Done!
            </Alert>}
            <br />
          </div>
        )
        )}
      </Box>

      <Typography align="center">{"Select a container file to use (.json or .yaml)"}</Typography>
    </Container >
  );
}

export default App;
