import { createSelector } from "@reduxjs/toolkit";
import { RootState } from "./store";

export const selectFirstClassifier = createSelector(
(state: RootState) => state.application.classifiers,
classifiers => {
    return classifiers.length > 0 ? classifiers[0]: undefined;
});