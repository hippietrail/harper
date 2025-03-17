import { Panel, PanelBody, TabPanel } from '@wordpress/components';
import React from 'react';
import type { IgnorableLintBox } from './Box';
import LintList from './LintList';
import LintSettingList from './LintSettingList';

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
									<LintSettingList />
								</PanelBody>
							</Panel>
						);
				}
			}}
		</TabPanel>
	);
}
