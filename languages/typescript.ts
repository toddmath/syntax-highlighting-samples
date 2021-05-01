module ModuleValidator {
  import checkChars = CharUtils.notWhiteSpace;
  export interface HasValidator<T> {
    validateValue(): Boolean;
  }

  class HasValidator<String> implements HasValidator<String> {
    /* Processed values */
    static validatedValue: Array<String> = ["", "aa"];
    private myValue: String;

    /**
     * Constructor for <code>HasValidator</code> class
     * @param value for <i>validation</i>
     */
    constructor(value: String) {
      this.myValue = value;
      HasValidator.validatedValue.push(value);
    }

    public validateValue(): Boolean {
      var resultValue: Boolean = checkChars(this.myValue);
      this.createInstance()
      return resultValue;
    }

    static createInstance(valueParameter: String): HasValidator {
      return new HasValidator(valueParameter);
    }
  }

  function globalFunction<TypeParameter>(value: TypeParameter) { //global function
    return 42;
  }

  HasValidator.createInstance(varUrl).validateValue();
}
#