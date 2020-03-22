import React from "react";
import { useTranslation } from "react-i18next";

export const HomePage: React.FC = () => {
  const { t } = useTranslation();

  return (
    <div className="row">
      <div className="col-12 col-md-4 order-sm-3">
        <div className="mt-3 pl-3">
          <h4>{t("homePage.login.title")}</h4>
          <div className="row">
            <div className="col-8">
              <button className="btn btn-block btn-social btn-google">
                <span className="fa fa-google"></span>{" "}
                {t("homePage.login.buttons.google")}
              </button>
            </div>
          </div>
        </div>
      </div>
      <div className="col-12 col-md-8">
        <img
          className="m-3 rounded-lg"
          src="/landing.jpg"
          width="100%"
          alt=""
        />
      </div>
    </div>
  );
};
