import { Header } from "./header";
import { HomePage } from "./homepage";
import React from "react";
import { useTranslation } from "react-i18next";

function App() {
  const { t } = useTranslation();

  return (
    <div className="App">
      <Header />
      <HomePage />
    </div>
  );
}

export default App;
