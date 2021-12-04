import { Button, TextInput } from "evergreen-ui";
import { Formik } from "formik";
import React from "react";
import { AccountSignUpModel } from "../../recoil/form/account/sign-up";

interface TemplateProps {
  name: string;
}

const Template: React.FC<TemplateProps> = (_props: TemplateProps) => {
  return (
    <div className="flex justify-center h-screen items-center">
      <form action=""></form>

      <Formik
        initialValues={{ email: "", password: "" }}
        validate={AccountSignUpModel.validate}
        onSubmit={(values, { setSubmitting }) => {
          setTimeout(() => {
            alert(JSON.stringify(values, null, 2));
            setSubmitting(false);
          }, 400);
        }}
      >
        {({ values, errors, touched, handleChange, handleBlur, handleSubmit, isSubmitting }) => (
          <form onSubmit={handleSubmit} className="grid grid-cols gap-4">
            <TextInput
              type="email"
              name="email"
              value={values.email}
              onChange={handleChange}
              onBlur={handleBlur}
            />
            {errors.email && touched.email && errors.email}
            <TextInput
              type="password"
              name="password"
              onChange={handleChange}
              onBlur={handleBlur}
              value={values.password}
            />
            {errors.password && touched.password && errors.password}
            <Button type="submit" disabled={isSubmitting} marginRight={16}>
              Submit
            </Button>
          </form>
        )}
      </Formik>
    </div>
  );
};

const SignUpPage: React.FC = () => {
  const _props: TemplateProps = {
    name: "",
  };

  return <Template {..._props} />;
};

export default SignUpPage;
