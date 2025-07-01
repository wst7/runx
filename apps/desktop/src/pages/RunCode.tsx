import { useCallback, useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import Editor from "@monaco-editor/react";
import debounce from "debounce-promise";
import Output from "@/components/Output";
import { LogOutput, LogOutputValueType } from "@/LogType";
import { useMount } from "ahooks";
import Sidebar, { Menu } from "@/components/Sidebar";
import Start from "@/assets/start.svg?react";
import Stop from "@/assets/stop.svg?react";
import useSettingsWebview from "@/hooks/useSettingsWebview";
import { RunXSelect } from "@/components/RunXSelect";

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
  const [output, setOutput] = useState<{ language: string, value: LogOutput[][] }>();
  const [code, setCode] = useState<string | undefined>(DefaultCode);
  const [theme, setTheme] = useState<string>("vs-dark");
  const [language, setLanguage] = useState<string>("typescript");
  const { openWindow } = useSettingsWebview();

  const runCode = useCallback(
    debounce(async (value?: string) => {
      if (!value) {
        setOutput({ language: language, value: [] });
        return;
      }
      try {
        const res = await invoke<{ language: string, value: LogOutput[][] }>("run_code_with_type", {
          code: value,
          language: language,
        });
        setOutput({ language: language, value: res.value });
      } catch (error) {
        setOutput({ language: language, value: [[{ type: LogOutputValueType.String, value: error as string }]] });
      }
    }, 500),
    [language]
  );

  const handleEditorChange = (value?: string) => {
    setCode(value);
    runCode(value);
  };

  useMount(() => {
    runCode(code);
  });

  useEffect(() => {
    if (language) {
      runCode(code);
    }
  }, [language]);

  const menus: Menu[] = [
    {
      icon: Start,
      title: "Run",
      onClick: () => {
        runCode(code);
      },
    },
    {
      icon: Stop,
      title: "Stop",
      onClick: () => {
        setOutput(undefined);
      },
    },
  ];

  const onSettingClick = () => {
    openWindow();
  };


  return (
    <main className="flex flex-row h-screen w-screen bg-[#1e1e1e]">
      <Sidebar menus={menus} onSettingClick={onSettingClick} />
      <div className="flex flex-1 flex-col w-full">
        <div className="flex bg-[#1e1e1e] h-[48px] text-white gap-2 items-center px-2">
          <RunXSelect
            value={language}
            placeholder="Select a language"
            onChange={setLanguage}
            items={[
              { value: "typescript", label: "TypeScript" },
              { value: "javascript", label: "JavaScript" },
              { value: "python", label: "Python" },
            ]}
          />
    
        </div>
        <div className="flex flex-row w-full flex-1">
          <Editor
            height="100%"
            width="50%"
            language={language}
            theme={theme}
            value={code}
            onChange={handleEditorChange}
          />
          <div className="h-full w-1/2 p-4 overflow-auto text-white">
            <Output logs={output} />
          </div>
        </div>
      </div>
    </main>
  );
}

export default App;
