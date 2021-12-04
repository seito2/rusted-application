import { atom } from "recoil";
import { AccountSignUpFormState } from "./type";

const initialState: AccountSignUpFormState = {
  email: "",
  password: "",
  confirmPassword: "",
};

export const state = atom({
  default: initialState,
  key: `form/account/sign-up`,
});
