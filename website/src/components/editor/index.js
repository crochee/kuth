import { Input } from 'antd';
import { useRef, useState, useEffect } from 'react';

const EditableCell = (props) => {
    const {
        content,
        onSave,
    } = props
    const [editing, setEditing] = useState(false);
    const inputRef = useRef(null);

    useEffect(() => {
        if (editing) {
            inputRef.current.focus({
                cursor: 'end',
            });
        }
    }, [editing]);

    const save = (e) => {
        e.preventDefault();
        let value = e.target.value;
        setEditing(true);
        if (value && value !== content) {
            onSave(value);
        }
        setEditing(false);
    };

    return editing ? (
        <Input ref={inputRef} allowClear defaultValue={content} onPressEnter={save} onBlur={save} />
    ) : (
        <div
            className="editable-cell-value-wrap"
            style={{
                minWidth: 24,
                height: 22,
            }}
            onClick={() => {
                setEditing(true);
            }}
        >
            {content}
        </div>
    );
};

export default EditableCell;