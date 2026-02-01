import type { INodeTypeBaseDescription, IVersionedNodeType } from 'n8n-workflow';
import { VersionedNodeType } from 'n8n-workflow';
import { PromptKaizenV1 } from './PromptKaizenV1.node';

export class PromptKaizen extends VersionedNodeType {
	constructor() {
		const baseDescription: INodeTypeBaseDescription = {
			displayName: 'Prompt Kaizen',
			name: 'promptKaizen',
		icon: 'file:raccoon.svg',
			group: ['transform'],
			description: 'Use prompts and submit feedback to Prompt Kaizen',
			defaultVersion: 1,
		};

		const nodeVersions: IVersionedNodeType['nodeVersions'] = {
			1: new PromptKaizenV1(baseDescription),
		};

		super(nodeVersions, baseDescription);
	}
}