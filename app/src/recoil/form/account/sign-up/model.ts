import { AccountSignUpFormState } from "./type";

/**
 * @package
 */
export class Model {
  static validate<T extends AccountSignUpFormState>(form: T) {
    const errors: Partial<AccountSignUpFormState> = {};

    if (!form.email) {
      errors.email = "Required";
    } else if (!/^[A-Z0-9._%+-]+@[A-Z0-9.-]+\.[A-Z]{2,}$/i.test(form.email)) {
      errors.email = "Invalid email address";
    }

    return errors;
  }
}
