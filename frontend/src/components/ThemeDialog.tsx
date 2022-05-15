import { useEffect, useState } from "react";
import axios, { AxiosResponse, AxiosError } from "axios";
import Button from "@mui/material/Button";
import Dialog from "@mui/material/Dialog";
import DialogActions from "@mui/material/DialogActions";
import DialogContent from "@mui/material/DialogContent";
import DialogContentText from "@mui/material/DialogContentText";
import DialogTitle from "@mui/material/DialogTitle";
import FormControl from "@mui/material/FormControl";
import MenuItem from "@mui/material/MenuItem";
import Select from "@mui/material/Select";
import { ThemeData } from "./Theme";
import InputLabel from "@mui/material/InputLabel";
import { SnackbarData, useSnackbar } from "./SnackbarContext";

const FORMATS = ["json", "xresources"] as const;
type Format = typeof FORMATS[number];

type ThemeModalProps = {
  data: ThemeData | null;
  open: boolean;
  onClose: () => void;
};

const ThemeDialog = ({ data, open, onClose }: ThemeModalProps) => {
  const { addSnack } = useSnackbar();
  const [format, setFormat] = useState<Format>("json");
  const [text, setText] = useState("Click fetch to preview theme");

  const fetchTheme = async (format: string) => {
    console.debug(`Fecthing ${data?.name} theme in ${format} format`);
    axios
      .get(`http://localhost:3001/theme/${data?.name}?format=${format}`, {
        responseType: "text",
        transformResponse: undefined,
        transitional: {
          silentJSONParsing: false,
        },
      })
      .then((res: AxiosResponse) => {
        let text =
          format === "json" ? JSON.stringify(res.data, null, 4) : res.data;
        setText(text);
      })
      .catch((err: AxiosError) => {
        addSnack({
          message: `${err.name}: ${err.message} (${err.code})`,
          severity: "error",
        } as SnackbarData);
      });
  };

  useEffect(() => {
    if (open === true) {
      fetchTheme(format);
    }
  }, [open]);

  return (
    <Dialog open={open} onClose={onClose} maxWidth="sm" fullWidth>
      <DialogTitle>{`${data?.name} (${format})`}</DialogTitle>
      <DialogContent
        sx={{ color: data?.foreground, backgroundColor: data?.background }}
      >
        <DialogContentText
          fontFamily="monospace"
          component="pre"
          sx={{ color: data?.foreground }}
        >
          {text.split("\\n").map((line, i) => (
            <div key={i}>{line}</div>
          ))}
        </DialogContentText>
      </DialogContent>
      <DialogActions
        sx={{
          paddingTop: "2vh",
          display: "flex",
          justifyContent: "space-between",
          alignItems: "center",
        }}
      >
        <FormControl sx={{ minWidth: 200 }}>
          <InputLabel>Format</InputLabel>
          <Select
            autoFocus
            defaultValue={"json"}
            value={format}
            onChange={(event) => {
              setFormat(event.target.value as Format);
              fetchTheme(event.target.value);
            }}
            label="Format"
          >
            {FORMATS.map((format, i) => (
              <MenuItem key={i} value={format}>
                {format}
              </MenuItem>
            ))}
          </Select>
        </FormControl>

        <Button
          variant="contained"
          color="success"
          onClick={() => fetchTheme(format)}
        >
          Fetch
        </Button>
      </DialogActions>
    </Dialog>
  );
};

export default ThemeDialog;
