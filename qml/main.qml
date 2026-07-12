import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import QtQuick.Window 2.15
import QtQuick.Dialogs

ApplicationWindow {
    id: window
    visible: true
    width: 1000
    height: 700
    title: "DOTA 2 Localization Changer"
    color: "#2C2F33"

    readonly property var app: appState

    property var heroData: []
    property var itemData: []
    property var presetList: []
    property var editingHeroIndex: -1

    Component.onCompleted: {
        refreshHeroes()
        refreshItems()
        refreshPresets()
    }

    function refreshHeroes() {
        try { heroData = JSON.parse(app.getHeroesJson()) } catch(e) { heroData = [] }
    }

    function refreshItems() {
        try { itemData = JSON.parse(app.getItemsJson()) } catch(e) { itemData = [] }
    }

    function refreshPresets() {
        try { presetList = JSON.parse(app.getPresetNamesJson()) } catch(e) { presetList = [] }
    }

    ColumnLayout {
        anchors.fill: parent
        anchors.margins: 10
        spacing: 10

        Label {
            text: "DOTA 2 Localization Changer"
            font.pointSize: 22
            font.bold: true
            color: "white"
            horizontalAlignment: Text.AlignHCenter
            Layout.fillWidth: true
            Layout.bottomMargin: 5
        }

        TabBar {
            id: tabBar
            Layout.fillWidth: true
            background: Rectangle { color: "#2C2F33" }

            TabButton {
                text: "Герои"
                background: Rectangle {
                    color: tabBar.currentIndex === 0 ? "#7289DA" : "#36393F"
                    radius: 4
                }
                contentItem: Text {
                    text: parent.text; color: "white"; font.bold: true
                    horizontalAlignment: Text.AlignHCenter; verticalAlignment: Text.AlignVCenter
                }
            }
            TabButton {
                text: "Предметы"
                background: Rectangle {
                    color: tabBar.currentIndex === 1 ? "#7289DA" : "#36393F"
                    radius: 4
                }
                contentItem: Text {
                    text: parent.text; color: "white"; font.bold: true
                    horizontalAlignment: Text.AlignHCenter; verticalAlignment: Text.AlignVCenter
                }
            }
        }

        StackLayout {
            currentIndex: tabBar.currentIndex
            Layout.fillWidth: true
            Layout.fillHeight: true

            Page {
                background: Rectangle { color: "#2C2F33" }
                ColumnLayout {
                    anchors.fill: parent
                    spacing: 8

                    TextField {
                        id: heroSearch
                        placeholderText: "Поиск..."
                        placeholderTextColor: "#888888"
                        Layout.preferredWidth: 300
                        Layout.alignment: Qt.AlignLeft
                        color: "white"
                        background: Rectangle { color: "#36393F"; radius: 4 }
                        onTextChanged: {
                            app.filterHeroes(text)
                            refreshHeroes()
                        }
                    }

                    ScrollView {
                        Layout.fillWidth: true
                        Layout.fillHeight: true
                        clip: true
                        background: Rectangle { color: "#36393F"; radius: 4 }

                        GridView {
                            id: heroGrid
                            model: heroData.length
                            property int minCellWidth: 150
                            cellWidth: Math.floor(width / Math.max(1, Math.floor(width / minCellWidth)))
                            cellHeight: 40
                            boundsBehavior: Flickable.StopAtBounds

                            delegate: Rectangle {
                                width: heroGrid.cellWidth - 8
                                height: heroGrid.cellHeight - 4
                                color: mouseArea.containsMouse ? "#40444B" : "#36393F"
                                radius: 4
                                x: 4; y: 2

                                Text {
                                    anchors.fill: parent
                                    anchors.margins: 4
                                    text: heroData[index] ? heroData[index].name || "" : ""
                                    color: heroData[index] && heroData[index].username ? "#7289DA" : "white"
                                    font.bold: heroData[index] && heroData[index].username ? true : false
                                    font.pointSize: 10
                                    elide: Text.ElideRight
                                    horizontalAlignment: Text.AlignHCenter
                                    verticalAlignment: Text.AlignVCenter
                                }

                                MouseArea {
                                    id: mouseArea
                                    anchors.fill: parent
                                    hoverEnabled: true
                                    cursorShape: Qt.PointingHandCursor
                                    onDoubleClicked: {
                                        if (heroData[index]) {
                                            editingHeroIndex = index
                                            openHeroEditDialog(index)
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            Page {
                background: Rectangle { color: "#2C2F33" }
                ColumnLayout {
                    anchors.fill: parent
                    spacing: 8

                    TextField {
                        id: itemSearch
                        placeholderText: "Поиск..."
                        placeholderTextColor: "#888888"
                        Layout.preferredWidth: 300
                        Layout.alignment: Qt.AlignLeft
                        color: "white"
                        background: Rectangle { color: "#36393F"; radius: 4 }
                        onTextChanged: {
                            app.filterItems(text)
                            refreshItems()
                        }
                    }

                    ScrollView {
                        Layout.fillWidth: true
                        Layout.fillHeight: true
                        clip: true
                        background: Rectangle { color: "#36393F"; radius: 4 }

                        ListView {
                            id: itemsList
                            anchors.fill: parent
                            model: itemData.length
                            boundsBehavior: Flickable.StopAtBounds

                            header: Rectangle {
                                width: itemsList.width
                                height: 30
                                color: "#2C2F33"
                                radius: 4
                                Item {
                                    anchors.left: parent.left; anchors.leftMargin: 10
                                    anchors.right: parent.horizontalCenter; anchors.rightMargin: 5
                                    anchors.top: parent.top; anchors.bottom: parent.bottom
                                    Label { text: "Имя"; color: "white"; font.bold: true; anchors.centerIn: parent }
                                }
                                Item {
                                    anchors.left: parent.horizontalCenter; anchors.leftMargin: 5
                                    anchors.right: parent.right; anchors.rightMargin: 10
                                    anchors.top: parent.top; anchors.bottom: parent.bottom
                                    Label { text: "Кастомное имя"; color: "white"; font.bold: true; anchors.centerIn: parent }
                                }
                            }

                            delegate: Rectangle {
                                width: itemsList.width
                                height: 35
                                color: mouseAreaItem.containsMouse ? "#40444B" : "#36393F"

                                Item {
                                    anchors.left: parent.left; anchors.leftMargin: 10
                                    anchors.right: parent.horizontalCenter; anchors.rightMargin: 5
                                    anchors.top: parent.top; anchors.bottom: parent.bottom
                                    Label {
                                        text: itemData[index] ? itemData[index].name || "" : ""
                                        color: "white"
                                        anchors.fill: parent
                                        verticalAlignment: Text.AlignVCenter
                                        elide: Text.ElideRight
                                    }
                                }
                                Item {
                                    anchors.left: parent.horizontalCenter; anchors.leftMargin: 5
                                    anchors.right: parent.right; anchors.rightMargin: 10
                                    anchors.top: parent.top; anchors.bottom: parent.bottom
                                    Label {
                                        text: itemData[index] ? (itemData[index].username || itemData[index].name) : ""
                                        color: itemData[index] && itemData[index].username ? "#7289DA" : "#888888"
                                        font.bold: itemData[index] && itemData[index].username ? true : false
                                        anchors.fill: parent
                                        verticalAlignment: Text.AlignVCenter
                                        elide: Text.ElideRight
                                    }
                                }

                                MouseArea {
                                    id: mouseAreaItem
                                    anchors.fill: parent
                                    hoverEnabled: true
                                    cursorShape: Qt.PointingHandCursor
                                    onDoubleClicked: {
                                        if (itemData[index]) openItemEditDialog(index)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        Label {
            id: statusLabel
            text: app.getStatusMessage()
            color: "#888888"
            font.pointSize: 9
            Layout.fillWidth: true
        }

        RowLayout {
            Layout.fillWidth: true
            spacing: 8

            Button {
                text: "Сохранить пресет"
                Layout.fillWidth: true
                contentItem: Text { text: parent.text; color: "white"; horizontalAlignment: Text.AlignHCenter }
                background: Rectangle { color: "#7289DA"; radius: 4 }
                onClicked: openSavePresetDialog()
            }
            Button {
                text: "Загрузить пресет"
                Layout.fillWidth: true
                contentItem: Text { text: parent.text; color: "white"; horizontalAlignment: Text.AlignHCenter }
                background: Rectangle { color: "#7289DA"; radius: 4 }
                onClicked: openLoadPresetDialog()
            }
            Button {
                text: "Папка пресетов"
                Layout.fillWidth: true
                contentItem: Text { text: parent.text; color: "white"; horizontalAlignment: Text.AlignHCenter }
                background: Rectangle { color: "#7289DA"; radius: 4 }
                onClicked: app.openPresetsFolder()
            }
        }

        RowLayout {
            Layout.fillWidth: true
            spacing: 8

            Button {
                text: "Путь к Dota 2"
                Layout.fillWidth: true
                contentItem: Text { text: parent.text; color: "white"; horizontalAlignment: Text.AlignHCenter }
                background: Rectangle { color: "#7289DA"; radius: 4 }
                onClicked: dotaPathDialog.open()
            }
            Button {
                text: "Сбросить"
                Layout.fillWidth: true
                contentItem: Text { text: parent.text; color: "white"; horizontalAlignment: Text.AlignHCenter }
                background: Rectangle { color: "#ED4245"; radius: 4 }
                onClicked: resetConfirmDialog.open()
            }
        }

        RowLayout {
            Layout.fillWidth: true
            spacing: 8

            Button {
                text: "Применить изменения"
                Layout.fillWidth: true
                implicitHeight: 32
                contentItem: Text {
                    text: parent.text
                    color: "white"
                    horizontalAlignment: Text.AlignHCenter
                    verticalAlignment: Text.AlignVCenter
                    font.bold: true
                    font.pointSize: 13
                }
                background: Rectangle { color: "#57F287"; radius: 4 }
                onClicked: {
                    app.saveChanges()
                    refreshHeroes()
                    refreshItems()
                }
            }
        }
    }

    Dialog {
        id: heroEditDialog
        modal: true
        title: "Редактирование героя"
        x: Math.round((window.width - width) / 2)
        y: Math.round((window.height - height) / 2)
        width: 800
        height: 500
        background: Rectangle { color: "#2C2F33"; radius: 8 }

        function save() {
            if (editingHeroIndex < 0) return
            var heroName = heroNameField.text.trim()
            var skillsArr = []
            for (var i = 0; i < skillsModel.count; i++) {
                skillsArr.push({ custom: skillsModel.get(i).custom })
            }
            app.editHero(editingHeroIndex, heroName, JSON.stringify(skillsArr))
            refreshHeroes()
        }

        ColumnLayout {
            anchors.fill: parent
            anchors { topMargin: 8; leftMargin: 10; rightMargin: 10; bottomMargin: 10 }
            spacing: 8

            Label { text: "Имя героя:"; color: "white"; font.bold: true }
            TextField {
                id: heroNameField
                Layout.fillWidth: true
                color: "white"
                background: Rectangle { color: "#36393F"; radius: 4 }
            }

            ColumnLayout {
                Layout.fillWidth: true
                Layout.fillHeight: true
                Label { text: "Скиллы"; color: "white"; font.bold: true }

                ListView {
                    id: skillsView
                    Layout.fillWidth: true
                    Layout.fillHeight: true
                    model: ListModel { id: skillsModel }
                    clip: true
                    boundsBehavior: Flickable.StopAtBounds

                    delegate: Rectangle {
                        width: skillsView.width
                        height: 30
                        color: "#36393F"
                        Item {
                            anchors.left: parent.left; anchors.leftMargin: 5
                            anchors.right: parent.horizontalCenter; anchors.rightMargin: 2
                            anchors.top: parent.top; anchors.bottom: parent.bottom
                            Label {
                                text: model.name
                                color: "white"
                                anchors.fill: parent
                                verticalAlignment: Text.AlignVCenter
                                elide: Text.ElideRight
                            }
                        }
                        Item {
                            anchors.left: parent.horizontalCenter; anchors.leftMargin: 2
                            anchors.right: parent.right; anchors.rightMargin: 5
                            anchors.top: parent.top; anchors.bottom: parent.bottom
                            TextField {
                                text: model.custom
                                color: "white"
                                placeholderText: model.name
                                placeholderTextColor: "#888888"
                                anchors.fill: parent
                                background: Rectangle { color: "#40444B"; radius: 4 }
                                onEditingFinished: skillsModel.set(index, { custom: text })
                            }
                        }
                    }
                }
            }

            RowLayout {
                Layout.fillWidth: true
                spacing: 8

                Item { Layout.fillWidth: true }

                Button {
                    text: "Сохранить"
                    implicitWidth: 100
                    contentItem: Text { text: parent.text; color: "white"; horizontalAlignment: Text.AlignHCenter; font.bold: true }
                    background: Rectangle { color: "#57F287"; radius: 4 }
                    onClicked: { heroEditDialog.save(); heroEditDialog.close() }
                }
                Button {
                    text: "Отмена"
                    implicitWidth: 100
                    contentItem: Text { text: parent.text; color: "white"; horizontalAlignment: Text.AlignHCenter }
                    background: Rectangle { color: "#ED4245"; radius: 4 }
                    onClicked: heroEditDialog.close()
                }
            }
        }
    }

    function openHeroEditDialog(index) {
        var hero = heroData[index]
        if (!hero) return
        heroNameField.text = hero.username || ""

        skillsModel.clear()
        if (hero.skills) {
            for (var i = 0; i < hero.skills.length; i++) {
                skillsModel.append({
                    name: hero.skills[i].name,
                    custom: hero.skills[i].username || ""
                })
            }
        }

        heroEditDialog.open()
    }

    Dialog {
        id: itemEditDialog
        modal: true
        title: "Редактирование предмета"
        x: Math.round((window.width - width) / 2)
        y: Math.round((window.height - height) / 2)
        width: 400
        height: 180
        background: Rectangle { color: "#2C2F33"; radius: 8 }

        property int editingItemIndex: -1

        function save() {
            if (editingItemIndex < 0) return
            app.editItem(editingItemIndex, itemNameField.text.trim())
            refreshItems()
        }

        ColumnLayout {
            anchors.fill: parent
            anchors { topMargin: 8; leftMargin: 15; rightMargin: 15; bottomMargin: 10 }
            spacing: 10

            Label { text: "Новое имя:"; color: "white"; font.pointSize: 12 }
            TextField {
                id: itemNameField
                Layout.fillWidth: true
                color: "white"
                background: Rectangle { color: "#36393F"; radius: 4 }
            }

            RowLayout {
                Layout.fillWidth: true
                spacing: 8

                Item { Layout.fillWidth: true }

                Button {
                    text: "Сохранить"
                    implicitWidth: 100
                    contentItem: Text { text: parent.text; color: "white"; horizontalAlignment: Text.AlignHCenter; font.bold: true }
                    background: Rectangle { color: "#57F287"; radius: 4 }
                    onClicked: { itemEditDialog.save(); itemEditDialog.close() }
                }
                Button {
                    text: "Отмена"
                    implicitWidth: 100
                    contentItem: Text { text: parent.text; color: "white"; horizontalAlignment: Text.AlignHCenter }
                    background: Rectangle { color: "#ED4245"; radius: 4 }
                    onClicked: itemEditDialog.close()
                }
            }
        }
    }

    function openItemEditDialog(index) {
        var item = itemData[index]
        if (!item) return
        itemEditDialog.editingItemIndex = index
        itemNameField.text = item.username || ""
        itemEditDialog.open()
    }

    Dialog {
        id: savePresetDialog
        modal: true
        title: "Сохранение пресета"
        x: Math.round((window.width - width) / 2)
        y: Math.round((window.height - height) / 2)
        width: 400
        height: 400
        background: Rectangle { color: "#2C2F33"; radius: 8 }

        function save() {
            var name = presetNameField.text.trim()
            if (name) {
                app.savePreset(name)
                refreshPresets()
            }
        }

        onOpened: {
            refreshPresets()
        }

        ColumnLayout {
            anchors.fill: parent
            anchors { topMargin: 8; leftMargin: 15; rightMargin: 15; bottomMargin: 10 }
            spacing: 8

            Label { text: "Имя нового пресета:"; color: "white"; font.pointSize: 12 }
            TextField {
                id: presetNameField
                Layout.fillWidth: true
                color: "white"
                background: Rectangle { color: "#36393F"; radius: 4 }
            }

            Label {
                text: "Существующие пресеты (нажмите чтобы перезаписать):"
                color: "#888888"; font.pointSize: 9
                visible: existingPresetsForSave.count > 0
            }

            ListView {
                id: existingPresetsForSave
                Layout.fillWidth: true
                Layout.fillHeight: true
                model: presetList
                clip: true
                boundsBehavior: Flickable.StopAtBounds

                delegate: ItemDelegate {
                    width: existingPresetsForSave.width
                    text: modelData || ""
                    hoverEnabled: true
                    contentItem: Text {
                        text: parent.text; color: "white"
                        font.pointSize: 10
                        verticalAlignment: Text.AlignVCenter
                    }
                    background: Rectangle {
                        color: parent.hovered ? "#40444B" : "#36393F"
                    }
                    onClicked: {
                        presetNameField.text = modelData
                    }
                }
            }

            RowLayout {
                Layout.fillWidth: true
                spacing: 8

                Item { Layout.fillWidth: true }

                Button {
                    text: "Сохранить"
                    implicitWidth: 100
                    contentItem: Text { text: parent.text; color: "white"; horizontalAlignment: Text.AlignHCenter; font.bold: true }
                    background: Rectangle { color: "#57F287"; radius: 4 }
                    onClicked: { savePresetDialog.save(); savePresetDialog.close() }
                }
                Button {
                    text: "Отмена"
                    implicitWidth: 100
                    contentItem: Text { text: parent.text; color: "white"; horizontalAlignment: Text.AlignHCenter }
                    background: Rectangle { color: "#ED4245"; radius: 4 }
                    onClicked: savePresetDialog.close()
                }
            }
        }
    }

    function openSavePresetDialog() {
        presetNameField.text = ""
        refreshPresets()
        savePresetDialog.open()
    }

    Dialog {
        id: loadPresetDialog
        modal: true
        title: "Загрузка пресета"
        x: Math.round((window.width - width) / 2)
        y: Math.round((window.height - height) / 2)
        width: 400
        height: 400
        background: Rectangle { color: "#2C2F33"; radius: 8 }

        onOpened: refreshPresets()

        function load() {
            if (presetListView.currentIndex >= 0) {
                var name = presetList[presetListView.currentIndex]
                if (name) {
                    app.loadPreset(name)
                    refreshHeroes()
                    refreshItems()
                    refreshPresets()
                }
            }
        }

        ColumnLayout {
            anchors.fill: parent
            anchors { topMargin: 8; leftMargin: 15; rightMargin: 15; bottomMargin: 10 }
            spacing: 8

            Label { text: "Выберите пресет:"; color: "white"; font.pointSize: 12 }

            ListView {
                id: presetListView
                Layout.fillWidth: true
                Layout.fillHeight: true
                model: presetList
                clip: true
                boundsBehavior: Flickable.StopAtBounds
                currentIndex: -1

                delegate: ItemDelegate {
                    width: presetListView.width
                    text: modelData || ""
                    highlighted: ListView.isCurrentItem
                    hoverEnabled: true
                    contentItem: Text {
                        text: parent.text; color: "white"
                        font.pointSize: 10
                        verticalAlignment: Text.AlignVCenter
                    }
                    background: Rectangle {
                        color: parent.highlighted ? "#7289DA" : (parent.hovered ? "#40444B" : "#36393F")
                    }
                    onClicked: presetListView.currentIndex = index
                }
            }

            RowLayout {
                Layout.fillWidth: true
                spacing: 8

                Item { Layout.fillWidth: true }

                Button {
                    text: "Открыть"
                    implicitWidth: 100
                    contentItem: Text { text: parent.text; color: "white"; horizontalAlignment: Text.AlignHCenter; font.bold: true }
                    background: Rectangle { color: "#7289DA"; radius: 4 }
                    onClicked: { loadPresetDialog.load(); loadPresetDialog.close() }
                }
                Button {
                    text: "Отмена"
                    implicitWidth: 100
                    contentItem: Text { text: parent.text; color: "white"; horizontalAlignment: Text.AlignHCenter }
                    background: Rectangle { color: "#ED4245"; radius: 4 }
                    onClicked: loadPresetDialog.close()
                }
            }
        }
    }

    function openLoadPresetDialog() {
        presetListView.currentIndex = -1
        refreshPresets()
        loadPresetDialog.open()
    }

    Dialog {
        id: resetConfirmDialog
        modal: true
        title: "Подтверждение"
        x: Math.round((window.width - width) / 2)
        y: Math.round((window.height - height) / 2)
        width: 350
        height: 150
        background: Rectangle { color: "#2C2F33"; radius: 8 }

        ColumnLayout {
            anchors.fill: parent
            anchors { topMargin: 8; leftMargin: 15; rightMargin: 15; bottomMargin: 10 }
            spacing: 8

            Label {
                Layout.fillWidth: true
                Layout.fillHeight: true
                text: "Вы уверены что хотите сбросить все настройки?"
                color: "white"; font.pointSize: 11; wrapMode: Text.WordWrap
                verticalAlignment: Text.AlignVCenter
                horizontalAlignment: Text.AlignHCenter
            }

            RowLayout {
                Layout.fillWidth: true
                spacing: 8

                Item { Layout.fillWidth: true }

                Button {
                    text: "Да"
                    implicitWidth: 80
                    contentItem: Text { text: parent.text; color: "white"; horizontalAlignment: Text.AlignHCenter; font.bold: true }
                    background: Rectangle { color: "#ED4245"; radius: 4 }
                    onClicked: {
                        app.resetAll()
                        refreshHeroes()
                        refreshItems()
                        resetConfirmDialog.close()
                    }
                }
                Button {
                    text: "Нет"
                    implicitWidth: 80
                    contentItem: Text { text: parent.text; color: "white"; horizontalAlignment: Text.AlignHCenter }
                    background: Rectangle { color: "#7289DA"; radius: 4 }
                    onClicked: resetConfirmDialog.close()
                }
            }
        }
    }

    FolderDialog {
        id: folderDialog
        title: "Выберите папку Dota 2"
        onAccepted: {
            var raw = folderDialog.selectedFolder.toString()
            var path = raw
            if (path.startsWith("file:///")) {
                path = path.substring(7)
                if (path.length >= 3 && path[0] === '/' && path[2] === ':') {
                    path = path.substring(1)
                }
            }
            dotaPathField.text = path
        }
    }

    Dialog {
        id: dotaPathDialog
        modal: true
        title: "Смена пути Dota 2"
        x: Math.round((window.width - width) / 2)
        y: Math.round((window.height - height) / 2)
        width: 550
        height: 180
        background: Rectangle { color: "#2C2F33"; radius: 8 }

        function save() {
            var path = dotaPathField.text.trim()
            if (path) app.setDotaPath(path)
        }

        ColumnLayout {
            anchors.fill: parent
            anchors { topMargin: 8; leftMargin: 15; rightMargin: 15; bottomMargin: 10 }
            spacing: 8

            Label { text: "Путь к корневой директории Dota 2:"; color: "white" }

            RowLayout {
                Layout.fillWidth: true
                spacing: 8

                TextField {
                    id: dotaPathField
                    Layout.fillWidth: true
                    color: "white"
                    placeholderText: "/path/to/dota 2 beta"
                    placeholderTextColor: "#888888"
                    background: Rectangle { color: "#36393F"; radius: 4 }
                    text: app.getDotaPath()
                }
                Button {
                    text: "Обзор..."
                    implicitWidth: 90
                    contentItem: Text { text: parent.text; color: "white"; horizontalAlignment: Text.AlignHCenter }
                    background: Rectangle { color: "#4E5D94"; radius: 4 }
                    onClicked: folderDialog.open()
                }
            }

            Item { Layout.fillHeight: true }

            RowLayout {
                Layout.fillWidth: true
                spacing: 8

                Item { Layout.fillWidth: true }

                Button {
                    text: "Сохранить"
                    implicitWidth: 100
                    contentItem: Text { text: parent.text; color: "white"; horizontalAlignment: Text.AlignHCenter; font.bold: true }
                    background: Rectangle { color: "#57F287"; radius: 4 }
                    onClicked: { dotaPathDialog.save(); dotaPathDialog.close() }
                }
                Button {
                    text: "Отмена"
                    implicitWidth: 100
                    contentItem: Text { text: parent.text; color: "white"; horizontalAlignment: Text.AlignHCenter }
                    background: Rectangle { color: "#ED4245"; radius: 4 }
                    onClicked: dotaPathDialog.close()
                }
            }
        }
    }
}
