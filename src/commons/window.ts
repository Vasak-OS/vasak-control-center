import { getCurrentWindow, LogicalSize } from "@tauri-apps/api/window";

export async function setWindowPorperties() {
  try {
    const thisWindow = await getCurrentWindow();
    const { height: screenHeight } = await window.screen;
    await thisWindow.setSize(new LogicalSize(300, screenHeight - 60));
  } catch (error) {
    console.error("[Panel Error] Error configurando la ventana:", error);
  }
}
