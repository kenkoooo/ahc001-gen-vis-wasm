import React, { useState } from 'react';
import './App.css';
import parse from 'html-react-parser';

function generate(seed: string): Promise<string> {
  return import("./tools/build").then(m => m.generate(seed));
}

function visualize(input: string, output: string): Promise<string | JSX.Element | JSX.Element[]> {
  return import("./tools/build").then(m => {
    try {
      const svgText = m.visualize(input, output);
      return parse(svgText);
    } catch (error) {
      return "Invalid";
    }
  });
}

function App() {
  const [seedText, setSeedText] = useState("1");
  const [inputText, setInputText] = useState("");
  const [outputText, setOutputText] = useState("");
  const [svgElement, setSvgElement] = useState<string | JSX.Element | JSX.Element[]>("");
  console.log(svgElement);
  return (
    <div>
      <div>
        Seed:
      <input type="text" onChange={(e) => setSeedText(e.target.value)} />
        <button onClick={() => {
          generate(seedText).then(text => {
            setInputText(text);
            setOutputText("");
          })
        }}>Generate</button>
      </div>
      <div>
        <div>
          Input:
        <textarea cols={50} rows={10} value={inputText} onChange={(e) => setInputText(e.target.value)} />
        </div>
        <div>
          Output:
        <textarea cols={50} rows={10} value={outputText} onChange={(e) => setOutputText(e.target.value)} />
        </div>
      </div>
      <div>
        <button onClick={() => visualize(inputText, outputText).then(s => setSvgElement(s))}>Visualize</button>
        <div style={{ width: 1000 }}>{svgElement}</div>
      </div>
    </div>
  );
}

export default App;
