import React from "react";
import ReactDOM from "react-dom/client";
import '@fontsource/roboto/300.css';
import '@fontsource/roboto/400.css';
import '@fontsource/roboto/500.css';
import '@fontsource/roboto/700.css';

import App from "./App";
import "./styles.css";
import ResponsiveAppBar from "./components/ResponsiveAppBar";
import { BrowserRouter, Route, Routes } from "react-router-dom";
import Browse from "./pages/Browse";
import Community from "./pages/Community";
import { ThemeProvider, createTheme } from "@mui/material";
import Package from "./pages/Package";

const darkTheme = createTheme({
  palette: {
    mode: 'dark',
  },
});

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <BrowserRouter >
      <ThemeProvider theme={darkTheme} >
        <ResponsiveAppBar />
        <Routes >
          <Route path="/" element={<App />} />
          <Route path="/browse" element={<Browse />} />
          <Route path="/community/:id" element={<Community />} />
          <Route path="/package/:namespace/:name" element={<Package />} />
        </Routes>
      </ThemeProvider>
    </BrowserRouter>
  </React.StrictMode>,
);
