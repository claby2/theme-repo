import { useEffect, useState } from "react";
import axios from "axios";
import Typography from "@mui/material/Typography";
import Card from "@mui/material/Card";
import CardContent from "@mui/material/CardContent";
import SquareIcon from "@mui/icons-material/Square";
import Box from "@mui/material/Box";

export type ThemeData = {
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
  selection_foreground: string;
  cursor: string;
  foreground: string;
  selection_background: string;
};

type ThemeProps = {
  name: string;
};

const Theme = ({ name }: ThemeProps) => {
  const [themeData, setThemeData] = useState<ThemeData | null>(null);

  useEffect(() => {
    axios.get(`http://localhost:3001/theme?${name}`).then((res) => {
      const data = res.data;
      const theme_data: ThemeData = {
        ...data,
      };
      setThemeData(theme_data);
    });
  }, []);

  console.log(themeData?.background);

  return (
    <Card elevation={4}>
      <CardContent sx={{ backgroundColor: themeData?.background }}>
        <Typography color={themeData?.foreground} variant="h6" marginBottom={1}>
          {name}
        </Typography>

        <Box>
          <SquareIcon htmlColor={themeData?.color0} />
          <SquareIcon htmlColor={themeData?.color1} />
          <SquareIcon htmlColor={themeData?.color2} />
          <SquareIcon htmlColor={themeData?.color3} />
          <SquareIcon htmlColor={themeData?.color4} />
          <SquareIcon htmlColor={themeData?.color5} />
          <SquareIcon htmlColor={themeData?.color6} />
          <SquareIcon htmlColor={themeData?.color7} />
        </Box>

        <Box>
          <SquareIcon htmlColor={themeData?.color8} />
          <SquareIcon htmlColor={themeData?.color9} />
          <SquareIcon htmlColor={themeData?.color10} />
          <SquareIcon htmlColor={themeData?.color11} />
          <SquareIcon htmlColor={themeData?.color12} />
          <SquareIcon htmlColor={themeData?.color13} />
          <SquareIcon htmlColor={themeData?.color14} />
          <SquareIcon htmlColor={themeData?.color15} />
        </Box>
      </CardContent>
    </Card>
  );
};

export default Theme;
