import useDialect from './useDialect';
import { SelectControl } from '@wordpress/components';
import { Dialect } from 'harper.js';
import React from 'react';

export default function DialectSelectRow() {
	const [dialect, setDialect] = useDialect();

	return (
		<div>
			<h3>Dialect</h3>
			<p>Choose which English dialect Harper should expect.</p>
			<SelectControl
				label="Dialect"
				value={dialect.toString()}
				options={[
					{
						label: 'American',
						value: Dialect.American.toString()
					},
					{
						label: 'Canadian',
						value: Dialect.Canadian.toString()
					},
					{
						label: 'Australian',
						value: Dialect.Australian.toString()
					},
					{
						label: 'British',
						value: Dialect.British.toString()
					}
				]}
				onChange={(value) => setDialect(parseInt(value))}
			></SelectControl>
		</div>
	);
}
