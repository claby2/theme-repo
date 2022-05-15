import { useEffect, useState } from "react";
import axios, { AxiosResponse, AxiosError } from "axios";
import Container from "@mui/material/Container";
import Box from "@mui/material/Box";
import Typography from "@mui/material/Typography";
import Theme, { ThemeData } from "./components/Theme";
import Grid from "@mui/material/Grid";
import { SnackbarData, useSnackbar } from "./components/SnackbarContext";
import Snackbar from "@mui/material/Snackbar";
import Alert from "@mui/material/Alert";

const App = () => {
  const { snack, removeSnack, addSnack } = useSnackbar();
  const [themes, setThemes] = useState<Array<ThemeData>>([]);

  useEffect(() => {
    console.debug("Fetching themes");
    axios
      .get("http://localhost:3001/themes")
      .then((res: AxiosResponse) => {
        setThemes(
          res.data.map((theme: object) => {
            return { ...theme } as ThemeData;
          })
        );
      })
      .catch((err: AxiosError) => {
        addSnack({
          message: `${err.name}: ${err.message} (${err.code})`,
          severity: "error",
        } as SnackbarData);
      });
  }, []);

  return (
    <>
      <Container maxWidth="lg">
        <Box sx={{ my: 4 }}>
          <Typography
            variant="h3"
            component="h1"
            fontWeight="bold"
            marginBottom="5vh"
          >
            Theme Repo
          </Typography>

          <Grid container spacing={2}>
            {themes.map((data, i) => (
              <Grid item xs={6} key={i}>
                <Theme data={data} />
              </Grid>
            ))}
          </Grid>
        </Box>
      </Container>
      <Snackbar
        open={snack !== null}
        onClose={() => removeSnack()}
        autoHideDuration={6000}
      >
        <Alert variant="filled" severity={snack?.severity}>
          {snack?.message}
        </Alert>
      </Snackbar>
    </>
  );
};

export default App;
