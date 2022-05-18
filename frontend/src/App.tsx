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
import CircularProgress from "@mui/material/CircularProgress";

const Loading = () => (
  <Box
    sx={{
      display: "flex",
      flexDirection: "column",
      width: "100%",
      alignItems: "center",
      justifyContent: "center",
    }}
  >
    <CircularProgress
      variant="indeterminate"
      size={80}
      disableShrink
      sx={{ marginBottom: "2vh" }}
    />
    <Typography>Fetching themes...</Typography>
  </Box>
);

const App = () => {
  const { snack, removeSnack, addSnack } = useSnackbar();
  const [themes, setThemes] = useState<Array<ThemeData>>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    console.debug("Fetching themes");
    axios
      .get(`${process.env.REACT_APP_BACKEND_URL}/themes`)
      .then((res: AxiosResponse) => {
        setThemes(
          res.data.map((theme: object) => {
            return { ...theme } as ThemeData;
          })
        );
        setLoading(false);
      })
      .catch((err: AxiosError) => {
        addSnack({
          message: `${err.name}: ${err.message} (${err.code})`,
          severity: "error",
        } as SnackbarData);
      });
  }, [addSnack]);

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

          {loading ? (
            <Loading />
          ) : (
            <Grid container spacing={2}>
              {themes.map((data, i) => (
                <Grid item xs={6} key={i}>
                  <Theme data={data} />
                </Grid>
              ))}
            </Grid>
          )}
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
