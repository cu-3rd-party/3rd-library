import { MaterialsSection } from "@/common/organisms";
import { MOCK_MATERIALS } from "@/mocks";

const MaterialsPage = () => {
  return (
    <div className="w-full px-4 py-6 xl:w-11/12 mx-auto max-w-screen-2xl">
      <MaterialsSection materials={MOCK_MATERIALS} />
    </div>
  );
};

export default MaterialsPage;
