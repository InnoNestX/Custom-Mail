import { writeFileSync } from "node:fs";
import { renderAppHtml } from "../legacy-ts/ui.ts";

let html = renderAppHtml({
  fromName: "___FROM_NAME___",
  fromEmail: "___FROM_EMAIL___",
  addressBook: [],
});

if (!html.includes("const ADDRESS_BOOK = [];")) {
  throw new Error("ADDRESS_BOOK injection point not found");
}

html = html.replace(
  "const ADDRESS_BOOK = [];",
  "const ADDRESS_BOOK = ___ADDRESS_BOOK___;",
);

writeFileSync("templates/app.html", html);
console.log("wrote templates/app.html", html.length, "bytes");
