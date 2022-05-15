import { useEffect, useState } from "react";
import axios from "axios";
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
import Box from "@mui/material/Box";
import InputLabel from "@mui/material/InputLabel";
import Typography from "@mui/material/Typography";

const FORMATS = ["json", "xresources"];

type ThemeModalProps = {
  data: ThemeData | null;
  open: boolean;
  onClose: () => void;
};

const ThemeDialog = ({ data, open, onClose }: ThemeModalProps) => {
  const [format, setFormat] = useState(FORMATS[0]);
  const [link, setLink] = useState("");
  const [text, setText] = useState("Click fetch to preview theme");

  const fetchTheme = async () => {
    axios
      .get(link, {
        responseType: "text",
        transformResponse: undefined,
        transitional: {
          silentJSONParsing: false,
        },
      })
      .then((res) => setText(JSON.stringify(res.data, null, 4)));
  };

  useEffect(() => {
    setLink(`http://localhost:3001/theme/${data?.name}?format=${format}`);
  }, [data, format]);

  return (
    <Dialog open={open} onClose={onClose} maxWidth="sm" fullWidth>
      <DialogTitle>{`${data?.name} (${format})`}</DialogTitle>
      <DialogContent
        sx={{ color: data?.foreground, backgroundColor: data?.background }}
      >
        <br />
        <DialogContentText component="pre" sx={{ color: data?.foreground }}>
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
            defaultValue={FORMATS[0]}
            value={format}
            onChange={(event) => {
              setFormat(event.target.value);
              fetchTheme();
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
          onClick={() => fetchTheme()}
        >
          Fetch
        </Button>
      </DialogActions>
    </Dialog>
  );
};

export default ThemeDialog;
