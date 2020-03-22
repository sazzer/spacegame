import { GameMenu } from "./gameMenu";
import React from "react";
import { UserMenu } from "./userMenu";
import { useTranslation } from "react-i18next";

/**
 * The header bar of the game
 */
export const Header: React.FC = () => {
  const { t } = useTranslation();

  return (
    <nav className="navbar navbar-expand-lg navbar-dark bg-dark">
      <a className="navbar-brand" href="#">
        {t("header.title")}
      </a>
      <button
        className="navbar-toggler"
        type="button"
        data-toggle="collapse"
        data-target="#navbarSupportedContent"
        aria-controls="navbarSupportedContent"
        aria-expanded="false"
        aria-label={t("header.toggleNavigation")}
      >
        <span className="navbar-toggler-icon"></span>
      </button>

      <div className="collapse navbar-collapse" id="navbarSupportedContent">
        <ul className="navbar-nav mr-auto">
          <GameMenu />
        </ul>

        <ul className="navbar-nav">
          <UserMenu />
        </ul>
      </div>
    </nav>
  );
};
