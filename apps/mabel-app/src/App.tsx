import { BrowserRouter, Routes, Route } from "react-router-dom";
import { DeviceProvider } from "./providers/DeviceProvider";
import AppLayout from "./components/layout/AppLayout";
import HomePage from "./pages/HomePage";
import AncPage from "./pages/AncPage";
import SoundPage from "./pages/SoundPage";
import SettingsPage from "./pages/SettingsPage";

function App() {
  return (
    <DeviceProvider>
      <BrowserRouter>
        <Routes>
          <Route element={<AppLayout />}>
            <Route path="/" element={<HomePage />} />
            <Route path="/anc" element={<AncPage />} />
            <Route path="/sound" element={<SoundPage />} />
            <Route path="/settings" element={<SettingsPage />} />
          </Route>
        </Routes>
      </BrowserRouter>
    </DeviceProvider>
  );
}

export default App;
