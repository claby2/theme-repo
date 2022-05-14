import { useEffect, useState } from "react";
import axios from "axios";
import Container from "@mui/material/Container";
import Box from "@mui/material/Box";
import Typography from "@mui/material/Typography";
import Theme, { ThemeData } from "./components/Theme";
import Grid from "@mui/material/Grid";

const App = () => {
  const [themes, setThemes] = useState<Array<ThemeData>>([]);

  useEffect(() => {
    axios.get("http://localhost:3001/themes").then((res) => {
      setThemes(
        res.data.map((theme: object) => {
          return { ...theme } as ThemeData;
        })
      );
    });
  }, []);

  return (
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
  );
};

export default App;
