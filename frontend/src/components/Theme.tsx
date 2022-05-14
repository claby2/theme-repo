import { useEffect, useState } from "react";
import axios from "axios";
import Typography from "@mui/material/Typography";
import Card from "@mui/material/Card";
import CardContent from "@mui/material/CardContent";
import CardActionArea from "@mui/material/CardActionArea";
import SquareIcon from "@mui/icons-material/Square";
import Box from "@mui/material/Box";
import ThemeDialog from "./ThemeDialog";

export type ThemeData = {
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
  selection_foreground: string;
  cursor: string;
  foreground: string;
  selection_background: string;
};

type ThemeProps = {
  data: ThemeData;
};

const Theme = ({ data }: ThemeProps) => {
  const [open, setOpen] = useState(false);

  return (
    <>
      <Card elevation={4}>
        <CardActionArea onClick={() => setOpen(true)}>
          <CardContent sx={{ backgroundColor: data.background }}>
            <Typography color={data.foreground} variant="h6" marginBottom={1}>
              {data.name}
            </Typography>

            <Box>
              <SquareIcon htmlColor={data.color0} />
              <SquareIcon htmlColor={data.color1} />
              <SquareIcon htmlColor={data.color2} />
              <SquareIcon htmlColor={data.color3} />
              <SquareIcon htmlColor={data.color4} />
              <SquareIcon htmlColor={data.color5} />
              <SquareIcon htmlColor={data.color6} />
              <SquareIcon htmlColor={data.color7} />
            </Box>

            <Box>
              <SquareIcon htmlColor={data.color8} />
              <SquareIcon htmlColor={data.color9} />
              <SquareIcon htmlColor={data.color10} />
              <SquareIcon htmlColor={data.color11} />
              <SquareIcon htmlColor={data.color12} />
              <SquareIcon htmlColor={data.color13} />
              <SquareIcon htmlColor={data.color14} />
              <SquareIcon htmlColor={data.color15} />
            </Box>
          </CardContent>
        </CardActionArea>
      </Card>
      <ThemeDialog data={data} open={open} onClose={() => setOpen(false)} />
    </>
  );
};

export default Theme;
