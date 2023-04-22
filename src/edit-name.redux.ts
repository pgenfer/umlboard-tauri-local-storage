import { createSlice, PayloadAction } from "@reduxjs/toolkit";
import { EditNameDto } from "./bindings/edit-name-dto";
import { classifiersLoaded } from "./app.redux";

type State = {
    currentName: string,
    editState: 'successful'|'canceled'|'none'
}

const initialState: State = {
    currentName: '',
    editState: 'none'
}

const classifierSlice = createSlice({
    name: 'classifier',
    initialState,
    reducers: {
        renamingClassifier(state, action: PayloadAction<EditNameDto>) {
            state.currentName = action.payload.newName;
            state.editState = 'none';
        },
        renameClassifier(state, action: PayloadAction<EditNameDto>) {
            // handled by backend
        },
        cancelClassifierRename(state) {
            // handled by backend            
        },
        classifierRenamed(state, action: PayloadAction<EditNameDto>) {
            // actually not necessary, but keep here in case the backend could alter the text
            state.currentName = action.payload.newName;
            state.editState = 'successful';
        },
        classifierRenameCanceled(state, action: PayloadAction<EditNameDto>) {
            // restore the name from core process
            state.currentName = action.payload.newName;
            state.editState = 'canceled';
        }
    }, 
    extraReducers: builder => {
        builder.addCase(classifiersLoaded, (state, action) => {
            if (action.payload.length > 0) {
                state.currentName = action.payload[0].name;
            } else {
                state.currentName = "no classifier loaded";
            }
        });
    }
});

export const classifierReducer = classifierSlice.reducer;
export const {renamingClassifier, renameClassifier, cancelClassifierRename} = classifierSlice.actions;