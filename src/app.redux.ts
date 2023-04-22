import { PayloadAction, createSlice } from "@reduxjs/toolkit";
import { ClassifierDto } from "./bindings/classifier-dto";

type State = {
    classifiers: ClassifierDto[]
}

const initialState: State = {
    classifiers: []
}

const appSlice = createSlice({
    name: 'application',
    initialState,
    reducers: {
        applicationReady(){
            // handled by backend
        },
        classifiersLoaded(state, action: PayloadAction<ClassifierDto[]>) {
            console.table('classifiers received ' + action.payload );
            state.classifiers = [...action.payload];
        }
    }
});

export const appReducer = appSlice.reducer;
export const { applicationReady, classifiersLoaded } = appSlice.actions;