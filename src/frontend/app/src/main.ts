import { bootstrapApp } from "./app";

async function bootstrap(): Promise<void> {
  const root = document.getElementById("app");
  if (!root) {
    throw new Error("missing #app root element");
  }

  await bootstrapApp(root);
}

void bootstrap();
