import React from "react";
import { useTranslation } from "react-i18next";

function App() {
  const { t } = useTranslation();

  return (
    <div className="App">
      <header className="App-header">{t("title")}</header>
    </div>
  );
}

export default App;
