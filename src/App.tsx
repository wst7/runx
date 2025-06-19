import { useCallback, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import Editor from "@monaco-editor/react";
import "./App.css";
import debounce from "debounce-promise";
import Output from "./components/Output";

const DefaultCode = `

const test = 'test&123';
console.log(test);
const list = [
  { name: 1, age: 18 },
  { name: 2, age: 19 },
];
console.log(list);


console.log(1, '3', undefined, true, false)
console.log([1, '3', undefined, true, false, null])

`;

function App() {
  const [output, setOutput] = useState<any[]>([]);
  const [code, setCode] = useState<string | undefined>(DefaultCode);
  const [theme, setTheme] = useState<string>("vs-dark");

  const runCode = useCallback(
    debounce(async (value?: string) => {
      if (!value) {
        setOutput([]);
        return;
      }
      try {
        const res = await invoke<any[]>("run_code_with_type", {
          code: value,
          language: "javascript",
        });
        setOutput(res);
      } catch (e) {
        setOutput([[e]]);
      }
    }, 500),
    []
  );

  const handleEditorChange = (value?: string) => {
    setCode(value);
    runCode(value);
  };

  return (
    <main className="flex flex-row h-screen w-screen bg-[#1e1e1e]">
      <Editor
        height="100vh"
        width="50%"
        defaultLanguage="javascript"
        theme={theme}
        value={code}
        onChange={handleEditorChange}
      />
      <div className="h-full w-1/2 p-4 overflow-auto text-white">
        <Output logs={output} />
      </div>
    </main>
  );
}

export default App;
