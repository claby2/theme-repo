import { useEffect, useState } from "react";
import axios, { AxiosResponse, AxiosError } from "axios";
import Dialog from "@mui/material/Dialog";
import DialogActions from "@mui/material/DialogActions";
import DialogContent from "@mui/material/DialogContent";
import DialogContentText from "@mui/material/DialogContentText";
import DialogTitle from "@mui/material/DialogTitle";
import FormControl from "@mui/material/FormControl";
import MenuItem from "@mui/material/MenuItem";
import Select from "@mui/material/Select";
import InputLabel from "@mui/material/InputLabel";
import Typography from "@mui/material/Typography";
import IconButton from "@mui/material/IconButton";
import CloseIcon from "@mui/icons-material/Close";
import ContentCopyIcon from "@mui/icons-material/ContentCopy";
import { SnackbarData, useSnackbar } from "./SnackbarContext";
import { ThemeData } from "./Theme";
import Tooltip from "@mui/material/Tooltip";

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
      <DialogTitle
        component="div"
        sx={{
          display: "flex",
          justifyContent: "space-between",
          alignItems: "center",
        }}
      >
        <Typography variant="h5">{`${data?.name} (${format})`}</Typography>
        <IconButton aria-label="close" onClick={onClose}>
          <CloseIcon />
        </IconButton>
      </DialogTitle>
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

        <Tooltip title="Copy to clipboard">
          <IconButton
            onClick={() => {
              navigator.clipboard.writeText(text);
              addSnack({
                message: "Copied to clipboard!",
                severity: "success",
              } as SnackbarData);
            }}
          >
            <ContentCopyIcon />
          </IconButton>
        </Tooltip>
      </DialogActions>
    </Dialog>
  );
};

export default ThemeDialog;
