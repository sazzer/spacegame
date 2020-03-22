import { Header } from "./header";
import React from "react";
import { useTranslation } from "react-i18next";

function App() {
  const { t } = useTranslation();

  return (
    <div className="App">
      <Header />
    </div>
  );
}

export default App;
