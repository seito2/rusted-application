import React from "react";

interface TemplateProps {
  name: string;
}

const Template: React.FC<TemplateProps> = (_props: TemplateProps) => {
  return <></>;
};

const SignUpPage: React.FC = () => {
  const _props: TemplateProps = {
    name: "",
  };

  return <Template {..._props} />;
};

export default SignUpPage;
