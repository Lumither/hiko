import { createTheme, ThemeProvider } from '@mui/material';
import React from 'react';

export default function MUIDarkLoader(
    {
        children
    }: {
        children: React.ReactNode
    }) {
    const dark = createTheme({
        palette: {
            mode: 'dark'
        }
    });
    return (
        <ThemeProvider theme={ dark }>
            { children }
        </ThemeProvider>
    );
}
