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
      .then((res) => setText(JSON.stringify(res.data)));
  };

  useEffect(() => {
    setLink(`http://localhost:3001/theme/${data?.name}?format=${format}`);
  }, [data, format]);

  return (
    <Dialog open={open} onClose={onClose}>
      <DialogTitle>{data?.name}</DialogTitle>
      <DialogContent sx={{ backgroundColor: data?.background }}>
        {text.split("\\n").map((line, i) => (
          <Typography key={i}>{line}</Typography>
        ))}
      </DialogContent>
      <DialogActions>
        <Box display="flex" justifyContent="space-between" alignItems="center">
          <FormControl sx={{ minWidth: 200 }}>
            <InputLabel>Format</InputLabel>
            <Select
              autoFocus
              defaultValue={FORMATS[0]}
              value={format}
              onChange={(event) => {
                setFormat(event.target.value);
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

          <Button onClick={() => fetchTheme()}>Fetch</Button>
        </Box>
      </DialogActions>
    </Dialog>
  );
};

export default ThemeDialog;
