import { useCallback, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import Editor from "@monaco-editor/react";
import "./App.css";
import debounce from "debounce-promise";
import Output from "./components/Output";

import { getCurrentWindow } from '@tauri-apps/api/window';
import { useMount } from "ahooks";
import { LogOutput, LogOutputValueType } from "./LogType";


const DefaultCode = `

const test: string = 'test&123';
console.log(test);
const list = [
  { name: 1, age: 18 },
  { name: 2, age: 19 },
];
console.log(list);


console.log(1, '3', undefined, true, false)
console.log([1, '3', undefined, true, false, null])

class Person {
  name: string;
  age: number;
  constructor(name: string, age: number) {
    this.name = name;
    this.age = age;
  }
}
class Student extends Person {
  school: string;
  constructor(name: string, age: number, school: string) {
    super(name, age);
    this.school = school;
  }
} 
console.log(new Student('John', 20, 'XYZ School'));

`;

function App() {
  const [output, setOutput] = useState<LogOutput[][]>([]);
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
          language: "typescript",
        });
        setOutput(res);
      } catch (error) {
        setOutput([[{ type: LogOutputValueType.String, value: error as string }]]);
      }
    }, 500),
    []
  );

  const handleEditorChange = (value?: string) => {
    setCode(value);
    runCode(value);
  };

  useMount(() => {
    getCurrentWindow().setTitleBarStyle("visible");
  });

  return (
    <main className="flex flex-row h-screen w-screen bg-[#1e1e1e]">
      <Editor
        height="100vh"
        width="50%"
        defaultLanguage="typescript"
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
