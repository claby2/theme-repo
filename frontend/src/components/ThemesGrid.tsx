import { useEffect, useState } from "preact/hooks";
import axios, { AxiosResponse } from "axios";

type ThemeData = {
  name: string;
  color0: string;
  color1: string;
  color2: string;
  color3: string;
  color4: string;
  color5: string;
  color6: string;
  color7: string;
  color8: string;
  color9: string;
  color10: string;
  color11: string;
  color12: string;
  color13: string;
  color14: string;
  color15: string;
  background: string;
  foreground: string;
};

type ThemeModalProps = {
  show: boolean;
  themeData: ThemeData;
  close: () => void;
};

const ThemeModal = ({ show, themeData, close }: ThemeModalProps) => {
  const [template, setTemplate] = useState<string>("json");
  const [templates, setTemplates] = useState<string[]>([]);
  const [text, setText] = useState<string>("");

  useEffect(() => {
    axios
      .get(
        `${import.meta.env.PUBLIC_BACKEND_URL}/themes/${
          themeData.name
        }?template=${template}`
      )
      .then((res: AxiosResponse) => {
        let text =
          template === "json" ? JSON.stringify(res.data, null, 4) : res.data;
        setText(text);
      });
  }, [template]);

  useEffect(() => {
    axios
      .get(`${import.meta.env.PUBLIC_BACKEND_URL}/templates`)
      .then((res: AxiosResponse) => {
        setTemplates(res.data);
      });
  }, []);

  if (show) {
    return (
      <>
        <div
          tabIndex={-1}
          aria-hidden={true}
          class="fixed h-full inset-0 bg-gray-900 opacity-50"
        ></div>
        <div class="fixed inset-0 h-full">
          <div class="flex justify-center content-center h-full">
            <div class="relative bg-gray-800 rounded-lg p-4 my-auto w-11/12 sm:w-2/3 lg:w-1/3">
              <div class="flex align-center justify-between w-full mb-4">
                <h3 class="font-bold text-2xl">{themeData.name}</h3>
                <button
                  class="bg-red-500 rounded p-2 transition hover:scale-110"
                  onClick={() => close()}
                >
                  Close
                </button>
              </div>
              <p
                class="whitespace-pre font-mono mb-4 rounded p-2 overflow-scroll"
                style={`background-color: ${themeData.background}; color: ${themeData.foreground}`}
              >
                {text}
              </p>
              <select
                value={template}
                onChange={(event) => {
                  setTemplate((event.target as HTMLSelectElement).value);
                }}
                class="bg-slate-700 rounded-md border-transparent ring-0 transition hover:scale-110"
              >
                {templates.map((template: string) => (
                  <option>{template}</option>
                ))}
              </select>
            </div>
          </div>
        </div>
      </>
    );
  } else {
    return <></>;
  }
};

type ThemePreviewProps = {
  themeData: ThemeData;
};

const ThemePreview = ({ themeData }: ThemePreviewProps) => {
  const [showModal, setShowModal] = useState(false);

  const firstColorRow = [
    themeData.color0,
    themeData.color1,
    themeData.color2,
    themeData.color3,
    themeData.color4,
    themeData.color5,
    themeData.color6,
    themeData.color7,
  ];
  const secondColorRow = [
    themeData.color8,
    themeData.color9,
    themeData.color10,
    themeData.color11,
    themeData.color12,
    themeData.color13,
    themeData.color14,
    themeData.color15,
  ];

  const generateSquares = (colorRow: string[]) => {
    return colorRow.map((color: string) => (
      <div class="w-6 h-6" style={`background-color: ${color}`}></div>
    ));
  };

  return (
    <>
      <div
        class="p-4 rounded-md border border-gray-700 shadow-md transition duration-75 hover:scale-105 hover:border-gray-300 hover:cursor-pointer"
        onClick={() => {
          setShowModal(true);
        }}
        style={`background-color: ${themeData.background}`}
      >
        <h2 class="font-bold text-2xl" style={`color: ${themeData.foreground}`}>
          {themeData.name}
        </h2>
        <div class="mt-4">
          <div class="flex">{generateSquares(firstColorRow)}</div>
          <div class="flex">
            <div class="flex">{generateSquares(secondColorRow)}</div>
          </div>
        </div>
      </div>
      <ThemeModal
        show={showModal}
        themeData={themeData}
        close={() => setShowModal(false)}
      />
    </>
  );
};

const ThemesGrid = () => {
  const [themes, setThemes] = useState<ThemeData[]>([]);

  useEffect(() => {
    axios
      .get(`${import.meta.env.PUBLIC_BACKEND_URL}/themes`)
      .then((res: AxiosResponse) => {
        setThemes(res.data.map((theme: object) => ({ ...theme } as ThemeData)));
      });
  }, []);

  return (
    <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
      {themes
        .sort((a, b) => a.name.localeCompare(b.name))
        .map((theme: ThemeData, i: number) => (
          <div key={i}>
            <ThemePreview themeData={theme} />
          </div>
        ))}
    </div>
  );
};

export default ThemesGrid;
