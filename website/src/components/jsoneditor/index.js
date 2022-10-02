import { useCallback, useEffect, useRef } from 'react';
import isEqual from 'lodash/isEqual';
import JSONEditor from 'jsoneditor';
import 'jsoneditor/dist/jsoneditor.css';
import './index.css';

const JsonEditor = props => {
    const {
        value,
        onChange,
        mode = 'view',
    } = props;
    const editorRef = useRef();
    const editorObj = useRef();

    const handleChange = useCallback((value) => {
        const currentValue = value === '' ? null : editorObj.current.get();
        onChange && onChange(currentValue);
    }, [onChange]);

    useEffect(() => {
        if (!editorObj.current) {
            editorObj.current = new JSONEditor(editorRef.current, {
                mode: mode || 'code',
                modes: ['code', 'form', 'text', 'tree', 'view', 'preview'],
                onChangeText: handleChange,
            })
            return
        }
        editorObj.current.setMode(mode);
    }, [mode, handleChange]);
    useEffect(() => {
        if (value && !isEqual(value, editorObj.current.get())) {
            editorObj.current.update(value);
        }
    }, [value])
    return <div className="kuth-json-editor" ref={editorRef} />
}

export default JsonEditor;