import { useState, createContext, useCallback, useContext } from "react";
import { AlertColor } from "@mui/material/Alert";

export type SnackbarData = {
  message: string;
  severity: AlertColor | undefined;
};

interface SnackbarContextValue {
  snack: SnackbarData | null;
  removeSnack: () => {};
  addSnack: (snack: SnackbarData) => {};
}

const SnackbarContext = createContext({
  snack: null,
  removeSnack: () => {},
  addSnack: (_: SnackbarData) => {},
} as SnackbarContextValue);

type SnackbarProviderProps = {
  children: React.ReactNode;
};

export const SnackbarProvider = ({ children }: SnackbarProviderProps) => {
  const [snack, setSnack] = useState<SnackbarData | null>(null);
  const removeSnack = () => setSnack(null);
  const addSnack = (snack: SnackbarData) => setSnack(snack);

  const contextValue = {
    snack,
    removeSnack: useCallback(() => removeSnack(), []),
    addSnack: useCallback((snack: SnackbarData) => addSnack(snack), []),
  } as SnackbarContextValue;

  return (
    <SnackbarContext.Provider value={contextValue}>
      {children}
    </SnackbarContext.Provider>
  );
};

export const useSnackbar = () => {
  return useContext(SnackbarContext);
};

export default SnackbarContext;
