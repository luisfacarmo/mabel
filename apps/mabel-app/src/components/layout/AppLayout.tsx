import Sidebar from "./Sidebar";
import TitleBar from "./TitleBar";
import AnimatedOutlet from "./AnimatedOutlet";

export default function AppLayout() {
  return (
    <div className="flex flex-col h-screen bg-bg overflow-hidden">
      <TitleBar />
      <div className="flex flex-1 overflow-hidden">
        <Sidebar />
        <main className="flex-1 overflow-y-auto p-8">
          <AnimatedOutlet />
        </main>
      </div>
    </div>
  );
}
