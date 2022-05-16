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

type ThemeModalProps = {
  data: ThemeData | null;
  open: boolean;
  onClose: () => void;
};

const ThemeDialog = ({ data, open, onClose }: ThemeModalProps) => {
  const { addSnack } = useSnackbar();
  const [templates, setTemplates] = useState<Array<string>>([]);
  const [template, setTemplate] = useState<string>("toml");
  const [text, setText] = useState("Click fetch to preview theme");

  useEffect(() => {
    const fetchTheme = async (template: string) => {
      console.debug(`Fetching ${data?.name} theme with ${template} template`);
      axios
        .get(
          `http://localhost:3001/themes/${data?.name}?template=${template}`,
          {
            responseType: "text",
            transformResponse: undefined,
            transitional: {
              silentJSONParsing: false,
            },
          }
        )
        .then((res: AxiosResponse) => {
          let text =
            template === "json" ? JSON.stringify(res.data, null, 4) : res.data;
          setText(text);
        })
        .catch((err: AxiosError) => {
          addSnack({
            message: `${err.name}: ${err.message} (${err.code})`,
            severity: "error",
          } as SnackbarData);
        });
    };

    if (open === true) {
      fetchTheme(template);
    }
  }, [open, template, addSnack, data?.name]);

  useEffect(() => {
    console.debug("Fetching templates");
    // Fetch templates
    axios
      .get("http://localhost:3001/templates")
      .then((res: AxiosResponse) => {
        setTemplates(res.data);
      })
      .catch((err: AxiosError) => {
        addSnack({
          message: `${err.name}: ${err.message} (${err.code})`,
          severity: "error",
        } as SnackbarData);
      });
  }, [addSnack]);

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
        <Typography variant="h5">{`${data?.name} (${template})`}</Typography>
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
            value={template}
            onChange={(event) => {
              setTemplate(event.target.value);
            }}
            label="Format"
          >
            {templates.map((template, i) => (
              <MenuItem key={i} value={template}>
                {template}
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
