<?php

/**
 * @file plugins/ojs/sourceright/SourcerightPlugin.php
 *
 * Thin OJS generic plugin wrapper for the Sourceright CLI/MCP core.
 */

import('lib.pkp.classes.plugins.GenericPlugin');

require_once __DIR__ . '/classes/SourcerightCliRunner.php';

class SourcerightPlugin extends GenericPlugin
{
    public function register($category, $path, $mainContextId = null)
    {
        $success = parent::register($category, $path, $mainContextId);

        if ($success && $this->getEnabled($mainContextId)) {
            HookRegistry::register('TemplateManager::display', [$this, 'addTemplateVars']);
        }

        return $success;
    }

    public function getDisplayName()
    {
        return __('plugins.generic.sourceright.displayName');
    }

    public function getDescription()
    {
        return __('plugins.generic.sourceright.description');
    }

    public function addTemplateVars($hookName, $args)
    {
        $templateMgr = $args[0];
        $request = Application::get()->getRequest();
        $context = $request ? $request->getContext() : null;
        $contextId = $context ? $context->getId() : CONTEXT_SITE;

        $templateMgr->assign([
            'sourcerightEnabled' => (bool) $this->getEnabled($contextId),
            'sourcerightWritesEnabled' => $this->writesExplicitlyEnabled($contextId),
        ]);

        return false;
    }

    public function screenSubmission($contextId, $submissionId, $workspaceRoot, $manuscriptLabel = 'manuscript')
    {
        $runner = new SourcerightCliRunner(
            $this->getConfiguredCliPath($contextId),
            $this->getConfiguredTimeout($contextId)
        );

        return $runner->screenOjsSubmission($workspaceRoot, (string) $submissionId, $manuscriptLabel);
    }

    public function previewExport($contextId, $workspaceRoot, $format = 'ris')
    {
        $runner = new SourcerightCliRunner(
            $this->getConfiguredCliPath($contextId),
            $this->getConfiguredTimeout($contextId)
        );

        return $runner->previewExport($workspaceRoot, $format);
    }

    public function writesExplicitlyEnabled($contextId)
    {
        return $this->getSetting($contextId, 'allowExplicitWrites') === true
            || $this->getSetting($contextId, 'allowExplicitWrites') === '1';
    }

    private function getConfiguredCliPath($contextId)
    {
        $configured = trim((string) $this->getSetting($contextId, 'sourcerightCliPath'));
        return $configured !== '' ? $configured : 'sourceright';
    }

    private function getConfiguredTimeout($contextId)
    {
        $configured = (int) $this->getSetting($contextId, 'sourcerightTimeoutSeconds');
        return $configured > 0 ? $configured : 30;
    }
}
