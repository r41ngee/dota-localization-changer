import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import QtQuick.Window 2.15

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
                                RowLayout {
                                    anchors.fill: parent
                                    anchors.leftMargin: 10; anchors.rightMargin: 10
                                    Label { text: "Имя"; color: "white"; font.bold: true; Layout.fillWidth: true }
                                    Label { text: "Кастомное имя"; color: "white"; font.bold: true; Layout.fillWidth: true }
                                }
                            }

                            delegate: Rectangle {
                                width: itemsList.width
                                height: 35
                                color: mouseAreaItem.containsMouse ? "#40444B" : "#36393F"

                                RowLayout {
                                    anchors.fill: parent
                                    anchors.leftMargin: 10; anchors.rightMargin: 10
                                    Label {
                                        text: itemData[index] ? itemData[index].name || "" : ""
                                        color: "white"; Layout.fillWidth: true
                                    }
                                    Label {
                                        text: itemData[index] ? (itemData[index].username || "N/A") : ""
                                        color: itemData[index] && itemData[index].username ? "#7289DA" : "#888888"
                                        font.bold: itemData[index] && itemData[index].username ? true : false
                                        Layout.fillWidth: true
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
                text: "Открыть папку пресетов"
                Layout.fillWidth: true
                contentItem: Text { text: parent.text; color: "white"; horizontalAlignment: Text.AlignHCenter }
                background: Rectangle { color: "#7289DA"; radius: 4 }
                onClicked: app.openPresetsFolder()
            }
            Button {
                text: "Сменить путь Dota 2"
                Layout.fillWidth: true
                contentItem: Text { text: parent.text; color: "white"; horizontalAlignment: Text.AlignHCenter }
                background: Rectangle { color: "#7289DA"; radius: 4 }
                onClicked: dotaPathDialog.open()
            }
            Button {
                text: "Сбросить настройки"
                Layout.fillWidth: true
                contentItem: Text { text: parent.text; color: "white"; horizontalAlignment: Text.AlignHCenter }
                background: Rectangle { color: "#ED4245"; radius: 4 }
                onClicked: resetConfirmDialog.open()
            }
            Button {
                text: "Сохранить изменения"
                Layout.fillWidth: true
                contentItem: Text { text: parent.text; color: "white"; horizontalAlignment: Text.AlignHCenter }
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
        standardButtons: Dialog.Save | Dialog.Cancel
        title: "Редактирование героя"
        x: Math.round((window.width - width) / 2)
        y: Math.round((window.height - height) / 2)
        width: 800
        height: 500
        background: Rectangle { color: "#2C2F33"; radius: 8 }

        onAccepted: {
            if (editingHeroIndex < 0) return
            var heroName = heroNameField.text.trim()
            var skillsArr = []
            for (var i = 0; i < skillsModel.count; i++) {
                skillsArr.push({ custom: skillsModel.get(i).custom })
            }
            var facetsArr = []
            for (var j = 0; j < facetsModel.count; j++) {
                facetsArr.push({ custom: facetsModel.get(j).custom })
            }
            app.editHero(editingHeroIndex, heroName, JSON.stringify(skillsArr), JSON.stringify(facetsArr))
            refreshHeroes()
        }

        ColumnLayout {
            anchors.fill: parent
            anchors.margins: 10
            spacing: 10

            Label { text: "Имя героя:"; color: "white"; font.bold: true }
            TextField {
                id: heroNameField
                Layout.fillWidth: true
                color: "white"
                background: Rectangle { color: "#36393F"; radius: 4 }
            }

            RowLayout {
                Layout.fillWidth: true
                Layout.fillHeight: true
                spacing: 10

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
                            RowLayout {
                                anchors.fill: parent
                                anchors.leftMargin: 5; anchors.rightMargin: 5
                                Label {
                                    text: model.name
                                    color: "white"
                                    Layout.fillWidth: true
                                }
                                TextField {
                                    text: model.custom
                                    color: "white"
                                    Layout.fillWidth: true
                                    Layout.minimumWidth: 80
                                    background: Rectangle { color: "#40444B"; radius: 4 }
                                    onEditingFinished: skillsModel.set(index, { custom: text })
                                }
                            }
                        }
                    }
                }

                ColumnLayout {
                    Layout.fillWidth: true
                    Layout.fillHeight: true
                    Label { text: "Аспекты"; color: "white"; font.bold: true }

                    ListView {
                        id: facetsView
                        Layout.fillWidth: true
                        Layout.fillHeight: true
                        model: ListModel { id: facetsModel }
                        clip: true
                        boundsBehavior: Flickable.StopAtBounds

                        delegate: Rectangle {
                            width: facetsView.width
                            height: 30
                            color: "#36393F"
                            RowLayout {
                                anchors.fill: parent
                                anchors.leftMargin: 5; anchors.rightMargin: 5
                                Label {
                                    text: model.name
                                    color: "white"
                                    Layout.fillWidth: true
                                }
                                TextField {
                                    text: model.custom
                                    color: "white"
                                    Layout.fillWidth: true
                                    Layout.minimumWidth: 80
                                    background: Rectangle { color: "#40444B"; radius: 4 }
                                    onEditingFinished: facetsModel.set(index, { custom: text })
                                }
                            }
                        }
                    }
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
                    custom: hero.skills[i].username || "N/A"
                })
            }
        }

        facetsModel.clear()
        if (hero.facets) {
            for (var j = 0; j < hero.facets.length; j++) {
                facetsModel.append({
                    name: hero.facets[j].name,
                    custom: hero.facets[j].username || "N/A"
                })
            }
        }

        heroEditDialog.open()
    }

    Dialog {
        id: itemEditDialog
        modal: true
        standardButtons: Dialog.Save | Dialog.Cancel
        title: "Редактирование предмета"
        x: Math.round((window.width - width) / 2)
        y: Math.round((window.height - height) / 2)
        width: 400
        height: 200
        background: Rectangle { color: "#2C2F33"; radius: 8 }

        property int editingItemIndex: -1

        onAccepted: {
            if (editingItemIndex < 0) return
            app.editItem(editingItemIndex, itemNameField.text.trim())
            refreshItems()
        }

        ColumnLayout {
            anchors.fill: parent
            anchors.margins: 20
            spacing: 15

            Label { text: "Новое имя:"; color: "white"; font.pointSize: 12 }
            TextField {
                id: itemNameField
                Layout.fillWidth: true
                color: "white"
                background: Rectangle { color: "#36393F"; radius: 4 }
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
        standardButtons: Dialog.Save | Dialog.Cancel
        title: "Сохранение пресета"
        x: Math.round((window.width - width) / 2)
        y: Math.round((window.height - height) / 2)
        width: 400
        height: 400
        background: Rectangle { color: "#2C2F33"; radius: 8 }

        onAccepted: {
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
            anchors.margins: 15
            spacing: 10

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
        standardButtons: Dialog.Open | Dialog.Cancel
        title: "Загрузка пресета"
        x: Math.round((window.width - width) / 2)
        y: Math.round((window.height - height) / 2)
        width: 400
        height: 400
        background: Rectangle { color: "#2C2F33"; radius: 8 }

        onOpened: refreshPresets()

        onAccepted: {
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
            anchors.margins: 15
            spacing: 10

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
        standardButtons: Dialog.Yes | Dialog.No
        title: "Подтверждение"
        x: Math.round((window.width - width) / 2)
        y: Math.round((window.height - height) / 2)
        width: 350
        height: 150
        background: Rectangle { color: "#2C2F33"; radius: 8 }

        onAccepted: {
            app.resetAll()
            refreshHeroes()
            refreshItems()
        }

        Label {
            anchors.centerIn: parent
            text: "Вы уверены что хотите сбросить все настройки?"
            color: "white"; font.pointSize: 11; wrapMode: Text.WordWrap
        }
    }

    Dialog {
        id: dotaPathDialog
        modal: true
        standardButtons: Dialog.Save | Dialog.Cancel
        title: "Смена пути Dota 2"
        x: Math.round((window.width - width) / 2)
        y: Math.round((window.height - height) / 2)
        width: 500
        height: 150
        background: Rectangle { color: "#2C2F33"; radius: 8 }

        onAccepted: {
            var path = dotaPathField.text.trim()
            if (path) app.setDotaPath(path)
        }

        ColumnLayout {
            anchors.fill: parent
            anchors.margins: 15
            spacing: 10

            Label { text: "Путь к корневой директории Dota 2:"; color: "white" }
            TextField {
                id: dotaPathField
                Layout.fillWidth: true
                color: "white"
                placeholderText: "/path/to/dota 2 beta"
                placeholderTextColor: "#888888"
                background: Rectangle { color: "#36393F"; radius: 4 }
                text: app.getDotaPath()
            }
        }
    }
}
