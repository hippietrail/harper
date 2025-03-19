import React from 'react';
import type { IgnorableLintBox } from './Box';
import LintSettingList from './LintSettingList';
import DialectSelectRow from './DialectSelectRow';
import { Panel, PanelBody, SelectControl, TabPanel } from '@wordpress/components';
import LintList from './LintList';

export default function SidebarTabContainer({
	lintBoxes,
	loading,
}: {
	lintBoxes: IgnorableLintBox[];
	loading: boolean;
}) {
	return (
		<TabPanel
			tabs={[
				{ name: 'errors', title: 'Errors' },
				{ name: 'settings', title: 'Settings' },
			]}
		>
			{(tab) => {
				switch (tab.name) {
					case 'errors':
						return <LintList lintBoxes={lintBoxes} loading={loading} />;
					case 'settings':
						return (
							<Panel>
								<PanelBody title="Rules">
									<DialectSelectRow />
									<LintSettingList />
								</PanelBody>
							</Panel>
						);
				}
			}}
		</TabPanel>
	);
}
